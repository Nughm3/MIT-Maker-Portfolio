clear; clc;
settings = initSettings();
settings.PAPR_threshold = 1.5; % Example threshold for PAPR reduction
data = randi([0 1], settings.numSubcarriers * 1000, 1); % Example binary data
txSignal = ofdm_transmitter(data, settings);
txSignal = reduce_papr(txSignal, settings); % Optional PAPR reduction
rxSignal = txSignal; % For simplicity, assume no channel effects

rxData = ofdm_receiver(rxSignal, settings);

numSymbols = length(rxData) / settings.numSubcarriers;
pilotSymbols = 1:10:numSymbols; % Example pilot positions, adjusted to be within bounds

% Equalize the received data using comm.LinearEqualizer
numTaps = 5;  % Number of taps for the equalizer
stepSize = 0.01;  % Step size for the equalizer adaptation
linEq = comm.LinearEqualizer('Algorithm', 'LMS', 'NumTaps', numTaps, 'StepSize', stepSize);

dataMatrix = reshape(rxData, settings.numSubcarriers, numSymbols);
H_est = mean(dataMatrix(:, pilotSymbols), 2);

equalizedData = zeros(size(dataMatrix));
for i = 1:size(dataMatrix, 1)
   equalizedData(i, :) = linEq(dataMatrix(i, :).', H_est(i)).';
end

rxData = equalizedData(:);

trackResults.status = '-';  
trackResults.carrFreq = inf(1, numSymbols);
trackResults.I_P = zeros(1, numSymbols);
trackResults.Q_P = zeros(1, numSymbols);
trackResults.dllDiscr = inf(1, numSymbols);
trackResults.dllDiscrFilt = inf(1, numSymbols);
trackResults.pllDiscr = inf(1, numSymbols);
trackResults.pllDiscrFilt = inf(1, numSymbols);
trackResults.remCarrPhase = inf(1, numSymbols);
trackResults.CNo.VSMValue = zeros(1, floor(numSymbols / settings.CNo.VSMinterval));
trackResults.CNo.VSMIndex = zeros(1, floor(numSymbols / settings.CNo.VSMinterval));
trackResults.satPos = zeros(3, numSymbols); 

for loopCnt = 1:numSymbols
   trackResults.carrFreq(loopCnt) = H_est(1);
   trackResults.I_P(loopCnt) = real(equalizedData(1, loopCnt));
   trackResults.Q_P(loopCnt) = imag(equalizedData(1, loopCnt));
end

keplerianElements.a = 26560e3; % Semi-major axis [m]
keplerianElements.e = 0.01; % Eccentricity
keplerianElements.i = deg2rad(55); % Inclination [rad]
keplerianElements.omega = deg2rad(0); % Argument of perigee [rad]
keplerianElements.Omega = deg2rad(0); % Right ascension of ascending node [rad]
keplerianElements.M0 = deg2rad(0); % Mean anomaly at epoch [rad]
t = (0:numSymbols-1) * settings.intTime; % Time vector

satPos = zeros(3, numSymbols);
for k = 1:numSymbols
   [satPos(:, k), ~] = calculateSatellitePosition(keplerianElements, t(k));
   trackResults.satPos(:, k) = satPos(:, k);
end

save('trackResults.mat', 'trackResults', 'settings');
plotResults();