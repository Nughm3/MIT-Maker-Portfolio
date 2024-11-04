function rxData = ofdm_receiver(rxSignal, settings)
    % OFDM Receiver
    % Inputs:
    %   rxSignal - Received OFDM signal
    %   settings - OFDM settings
    % Outputs:
    %   rxData - Recovered data stream (binary)

    % Parameters
    numSubcarriers = settings.numSubcarriers;
    cpLength = settings.cpLength;
    numSymbols = length(rxSignal) / (numSubcarriers + cpLength);

    % Reshape received signal into OFDM symbols
    rxSymbols = reshape(rxSignal, numSubcarriers + cpLength, numSymbols);

    % Remove cyclic prefix
    rxSymbols = rxSymbols(cpLength+1:end, :);

    % Perform FFT
    fftData = fft(rxSymbols, numSubcarriers);

    % Reshape to a single stream of data
    rxData = fftData(:);
end