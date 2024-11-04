function txSignal = ofdm_transmitter(data, settings)
    % OFDM Transmitter
    % Inputs:
    %   data - Input data stream (binary)
    %   settings - OFDM settings
    % Outputs:
    %   txSignal - Transmitted OFDM signal

    % Parameters
    numSubcarriers = settings.numSubcarriers;
    cpLength = settings.cpLength;
    numSymbols = length(data) / numSubcarriers;

    % Reshape data into subcarriers
    dataMatrix = reshape(data, numSubcarriers, numSymbols);

    % Perform IFFT
    ifftData = ifft(dataMatrix, numSubcarriers);

    % Add cyclic prefix
    ofdmSymbols = [ifftData(end-cpLength+1:end, :); ifftData];

    % Reshape to a single stream of IQ samples
    txSignal = ofdmSymbols(:);
end