function generate_ofdm_data()
    numSubcarriers = 2048; 
    cpLength = 512; 
    numSymbols = 1000;  
    samplingFreq = 26e6; 
    fileName = 'synthetic_ofdm_signal.bin'; 

    % Generate random binary data and map to QPSK
    data = randi([0 1], numSubcarriers, numSymbols) * 2 - 1; 
    ifftData = ifft(data, numSubcarriers); 

    % Add cyclic prefix
    ofdmSymbols = [ifftData(end-cpLength+1:end, :); ifftData];

    % Serialize OFDM symbols
    iqData = ofdmSymbols(:);

    % Add noise
    iqData = iqData + 0.1 * (randn(size(iqData)) + 1i * randn(size(iqData)));

    % Save to file
    fid = fopen(fileName, 'wb');
    fwrite(fid, iqData, 'float32');
    fclose(fid);

    fprintf('Synthetic OFDM data generated and saved to %s\n', fileName);
end