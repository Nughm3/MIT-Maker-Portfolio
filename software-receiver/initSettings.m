function settings = initSettings()
    % Initialize and save settings for processing OFDM signals.

    %% Processing settings
    settings.msToProcess = 1000;  % [ms]
    settings.numberOfChannels = 1;
    settings.skipNumberOfBytes = 0;  % Set to 0 for testing

    %% Raw signal file name and other parameters
    settings.fileName = 'synthetic_ofdm_signal.bin';  % Path to synthetic OFDM data
    settings.dataType = 'float32';  % Data type: 'float32' for 32-bit float
    settings.fileType = 2;  % File type: 2 for I/Q samples

    %% Signal characteristics
    settings.IF = 5e6;  % Intermediate frequency [Hz]
    settings.samplingFreq = 26e6;  % Sampling frequency [Hz]
    settings.numSubcarriers = 2048;  % Number of OFDM subcarriers
    settings.cpLength = 512;  % Cyclic prefix length

    %% Acquisition settings
    settings.skipAcquisition = 0;
    settings.acqSearchBand = 7000;  % Search band [Hz]
    settings.acqThreshold = 1.5;  % Acquisition threshold

    %% Tracking loop settings
    settings.dllDampingRatio = 0.7;
    settings.dllNoiseBandwidth = 1.5;  % [Hz]
    settings.dllCorrelatorSpacing = 0.5;  % [chips]
    settings.pllDampingRatio = 0.7;
    settings.pllNoiseBandwidth = 20;  % [Hz]
    settings.intTime = 0.001;  % Integration time [s]

    %% Navigation solution settings
    settings.navSolPeriod = 500;  % [ms]
    settings.elevationMask = 5;  % Elevation mask [degrees]
    settings.useTropCorr = 1;  % Tropospheric correction: 0 - Off, 1 - On

    %% Plot settings
    settings.plotTracking = 1;  % Plot tracking results: 0 - Off, 1 - On

    %% Constants
    settings.c = 299792458;  % Speed of light [m/s]
    settings.startOffset = 68.802;  % Initial signal travel time [ms]

    %% CNo Settings
    settings.CNo.accTime = 0.001;  % Accumulation interval in tracking [s]
    settings.CNo.VSMinterval = 40;  % VSM interval for computing C/No [ms]

    %% Additional required fields
    settings.codeFreqBasis = 1.023e6; % Frequency basis for code [Hz]
    settings.codeLength = 1023; % Length of the code
    settings.acqSatelliteList = 1:32; % List of PRNs to search for
    settings.resamplingThreshold = 16e6; % Threshold for resampling
    settings.resamplingflag = 1; % Flag for resampling
end