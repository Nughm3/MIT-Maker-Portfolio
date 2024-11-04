function [trackResults, channel] = tracking(fid, channel, settings)
% Performs OFDM tracking for the given data file.
%
% [trackResults, channel] = tracking(fid, channel, settings)
%
%   Inputs:
%       fid             - file identifier of the signal record.
%       channel         - Not used in this simplified version.
%       settings        - Receiver settings.
%   Outputs:
%       trackResults    - Tracking results (structure array).

%% Initialize result structure
trackResults.status = '-';  % No tracked signal, or lost lock
trackResults.absoluteSample = zeros(1, settings.msToProcess);
trackResults.carrFreq = inf(1, settings.msToProcess);
trackResults.I_P = zeros(1, settings.msToProcess);
trackResults.Q_P = zeros(1, settings.msToProcess);
trackResults.dllDiscr = inf(1, settings.msToProcess);
trackResults.dllDiscrFilt = inf(1, settings.msToProcess);
trackResults.pllDiscr = inf(1, settings.msToProcess);
trackResults.pllDiscrFilt = inf(1, settings.msToProcess);
trackResults.remCarrPhase = inf(1, settings.msToProcess);
trackResults.CNo.VSMValue = zeros(1, floor(settings.msToProcess / settings.CNo.VSMinterval));
trackResults.CNo.VSMIndex = zeros(1, floor(settings.msToProcess / settings.CNo.VSMinterval));
trackResults.satPos = zeros(3, settings.msToProcess); % Add satellite position
trackResults = repmat(trackResults, 1, settings.numberOfChannels);

%% Initialize tracking variables
codePeriods = settings.msToProcess;
PDIcode = settings.intTime;
[tau1code, tau2code] = calcLoopCoef(settings.dllNoiseBandwidth, settings.dllDampingRatio, 1.0);
PDIcarr = settings.intTime;
[tau1carr, tau2carr] = calcLoopCoef(settings.pllNoiseBandwidth, settings.pllDampingRatio, 0.25);
hwb = waitbar(0, 'Tracking...');
CNoPos = get(hwb, 'Position');
set(hwb, 'Position', [CNoPos(1), CNoPos(2), CNoPos(3), 90], 'Visible', 'on');
dataAdaptCoeff = 2;

% Initialize loop-specific variables
oldCarrNco = 0.0;
oldCarrError = 0.0;
oldCodeNco = 0.0;
oldCodeError = 0.0;
remCarrPhase = 0.0;
vsmCnt = 0;
CNo = 0;

% Define Keplerian elements for the satellite (example values)
keplerianElements.a = 26560e3; % Semi-major axis [m]
keplerianElements.e = 0.01; % Eccentricity
keplerianElements.i = deg2rad(55); % Inclination [rad]
keplerianElements.omega = deg2rad(0); % Argument of perigee [rad]
keplerianElements.Omega = deg2rad(0); % Right ascension of ascending node [rad]
keplerianElements.M0 = deg2rad(0); % Mean anomaly at epoch [rad]

%% Start processing
for channelNr = 1:settings.numberOfChannels
    %% Read the entire data file
    fseek(fid, settings.skipNumberOfBytes, 'bof');
    [rawSignal, samplesRead] = fread(fid, inf, settings.dataType);
    rawSignal = rawSignal(:);

    if samplesRead == 0
        disp('No data read from the file, exiting!');
        fclose(fid);
        return;
    end

    %% OFDM Demodulation
    demodSymbols = ofdm_demodulation(rawSignal, settings);

    %% Extract Metrics
    [carrFreq, I_P, Q_P] = extractMetrics(demodSymbols);

    %% Ensure Metrics are Scalars
    carrFreq = carrFreq(1);
    I_P = I_P(1);
    Q_P = Q_P(1);

    %% Save tracking results
    trackResults(channelNr).carrFreq(1) = carrFreq;
    trackResults(channelNr).I_P(1) = I_P;
    trackResults(channelNr).Q_P(1) = Q_P;

    %% Calculate Discriminators and Filters
    for loopCnt = 1:codePeriods
        if rem(loopCnt, 50) == 0
            trackingStatus = sprintf('Tracking: Ch %d of %d\nCompleted %d of %d msec\nC/No: %s (dB-Hz)', ...
                                     channelNr, settings.numberOfChannels, loopCnt, codePeriods, int2str(CNo));
            waitbar(loopCnt / codePeriods, hwb, trackingStatus);
        end

        % Calculate PLL error and update carrier NCO
        carrError = atan(Q_P / I_P) / (2.0 * pi);
        carrNco = oldCarrNco + (tau2carr / tau1carr) * (carrError - oldCarrError) + carrError * (PDIcarr / tau1carr);
        oldCarrNco = carrNco;
        oldCarrError = carrError;
        trackResults(channelNr).pllDiscr(loopCnt) = carrError;
        trackResults(channelNr).pllDiscrFilt(loopCnt) = carrNco;

        % Calculate DLL error and update code NCO
        codeError = calculateCodeError(demodSymbols);
        codeError = codeError(1); % Ensure codeError is a scalar
        codeNco = oldCodeNco + (tau2code / tau1code) * (codeError - oldCodeError) + codeError * (PDIcode / tau1code);
        oldCodeNco = codeNco;
        oldCodeError = codeError;
        trackResults(channelNr).dllDiscr(loopCnt) = codeError;
        trackResults(channelNr).dllDiscrFilt(loopCnt) = codeNco;

        % Save results
        trackResults(channelNr).remCarrPhase(loopCnt) = remCarrPhase;
        trackResults(channelNr).I_P(loopCnt) = I_P;
        trackResults(channelNr).Q_P(loopCnt) = Q_P;

        if rem(loopCnt, settings.CNo.VSMinterval) == 0
            vsmCnt = vsmCnt + 1;
            CNoValue = CNoVSM(trackResults(channelNr).I_P(loopCnt-settings.CNo.VSMinterval+1:loopCnt), ...
                              trackResults(channelNr).Q_P(loopCnt-settings.CNo.VSMinterval+1:loopCnt), settings.CNo.accTime);
            trackResults(channelNr).CNo.VSMValue(vsmCnt) = CNoValue;
            trackResults(channelNr).CNo.VSMIndex(vsmCnt) = loopCnt;
            CNo = int2str(CNoValue);
        end

        %% Calculate Satellite Position
        t = loopCnt * settings.intTime; % Time since epoch [s]
        [satPos, ~] = calculateSatellitePosition(keplerianElements, t);
        trackResults(channelNr).satPos(:, loopCnt) = satPos;
    end

    trackResults(channelNr).status = 'Tracking';
end

close(hwb);

%% Save results to a .mat file
save('trackResults.mat', 'trackResults', 'settings');
end