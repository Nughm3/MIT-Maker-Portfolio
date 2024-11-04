function plotResults()
    load('trackResults.mat', 'trackResults', 'settings');
    figure;
    subplot(1, 1, 1);
    plot(trackResults.carrFreq);
    title('Carrier Frequency');
    xlabel('Time (ms)');
    ylabel('Frequency (Hz)');
    grid on;

    figure;
    subplot(1, 1, 1);
    plot(trackResults.I_P);
    title('In-phase Prompt Correlator');
    xlabel('Time (ms)');
    ylabel('Amplitude');
    grid on;

    figure;
    subplot(1, 1, 1);
    plot(trackResults.Q_P);
    title('Quadrature Prompt Correlator');
    xlabel('Time (ms)');
    ylabel('Amplitude');
    grid on;

    figure;
    subplot(1, 1, 1);
    plot(trackResults.CNo.VSMIndex, trackResults.CNo.VSMValue);
    title('C/No');
    xlabel('Time (ms)');
    ylabel('C/No (dB-Hz)');
    grid on;

    figure;
    subplot(3, 1, 1);
    plot(trackResults.satPos(1, :));
    title('Satellite Position X');
    xlabel('Time (ms)');
    ylabel('X (m)');
    grid on;

    subplot(3, 1, 2);
    plot(trackResults.satPos(2, :));
    title('Satellite Position Y');
    xlabel('Time (ms)');
    ylabel('Y (m)');
    grid on;

    subplot(3, 1, 3);
    plot(trackResults.satPos(3, :));
    title('Satellite Position Z');
    xlabel('Time (ms)');
    ylabel('Z (m)');
    grid on;
end