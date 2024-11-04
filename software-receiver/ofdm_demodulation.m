function demodSymbols = ofdm_demodulation(iqData, settings)
    % Perform OFDM demodulation on the provided IQ data.

    % Parameters
    numSubcarriers = settings.numSubcarriers;  % Number of OFDM subcarriers
    cpLength = settings.cpLength;  % Cyclic prefix length

    % Reshape the data into OFDM symbols
    numSymbols = floor(length(iqData) / (numSubcarriers + cpLength));
    ofdmSymbols = reshape(iqData(1:numSymbols * (numSubcarriers + cpLength)), ...
                          numSubcarriers + cpLength, numSymbols);

    % Remove cyclic prefix
    ofdmSymbols = ofdmSymbols(cpLength+1:end, :);

    % Perform FFT to demodulate
    demodSymbols = fft(ofdmSymbols, numSubcarriers);

    % Further processing to extract data...
end