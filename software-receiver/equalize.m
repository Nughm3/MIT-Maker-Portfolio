function equalizedData = equalize(rxData, pilotSymbols, settings)
    % Equalize the received OFDM data using pilot symbols
    % Inputs:
    %   rxData - Received OFDM data
    %   pilotSymbols - Known pilot symbols
    %   settings - OFDM settings
    % Outputs:
    %   equalizedData - Equalized OFDM data

    % Parameters
    numSubcarriers = settings.numSubcarriers;
    numSymbols = length(rxData) / numSubcarriers;

    % Reshape received data into subcarriers
    dataMatrix = reshape(rxData, numSubcarriers, numSymbols);

    % Ensure pilotSymbols indices are within bounds
    pilotSymbols = pilotSymbols(pilotSymbols <= numSymbols);

    % Channel estimation using pilot symbols
    H_est = mean(dataMatrix(:, pilotSymbols), 2);

    % Equalize the data
    equalizedData = dataMatrix ./ H_est;

    % Reshape to a single stream of equalized data
    equalizedData = equalizedData(:);
end