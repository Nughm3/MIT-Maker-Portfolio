function acqResults = acquisition(longSignal, settings)
    % Perform acquisition on the collected Starlink data.
    
    %% Signal Conditioning
    % Comment out the resampling step if not necessary
    % if (settings.samplingFreq > settings.resamplingThreshold && settings.resamplingflag == 1)
    %     longSignal = resampleSignal(longSignal, settings);
    % end

    %% Initialization
    samplesPerCode = round(settings.samplingFreq / (settings.codeFreqBasis / settings.codeLength));
    signal1 = longSignal(1:samplesPerCode);
    signal2 = longSignal(samplesPerCode + 1:2 * samplesPerCode);
    signal0DC = longSignal - mean(longSignal);
    ts = 1 / settings.samplingFreq;
    phasePoints = (0:(samplesPerCode - 1)) * 2 * pi * ts;
    numberOfFrqBins = round(settings.acqSearchBand * 2 / 500) + 1;

    %% Generate Code Table (for Starlink if needed)
    caCodesTable = makeCaTable(settings);
    results = zeros(numberOfFrqBins, samplesPerCode);
    frqBins = zeros(1, numberOfFrqBins);
    acqResults.carrFreq = zeros(1, 32);
    acqResults.codePhase = zeros(1, 32);
    acqResults.peakMetric = zeros(1, 32);
    fprintf('(');
    for PRN = settings.acqSatelliteList
        caCodeFreqDom = conj(fft(caCodesTable(PRN, :)));
        for frqBinIndex = 1:numberOfFrqBins
            frqBins(frqBinIndex) = settings.IF - settings.acqSearchBand + 0.5e3 * (frqBinIndex - 1);
            sigCarr = exp(1i * frqBins(frqBinIndex) * phasePoints);
            I1 = real(sigCarr .* signal1);
            Q1 = imag(sigCarr .* signal1);
            I2 = real(sigCarr .* signal2);
            Q2 = imag(sigCarr .* signal2);
            IQfreqDom1 = fft(I1 + 1i * Q1);
            IQfreqDom2 = fft(I2 + 1i * Q2);
            convCodeIQ1 = IQfreqDom1 .* caCodeFreqDom;
            convCodeIQ2 = IQfreqDom2 .* caCodeFreqDom;
            acqRes1 = abs(ifft(convCodeIQ1));
            acqRes2 = abs(ifft(convCodeIQ2));
            if (max(acqRes1) > max(acqRes2))
                results(frqBinIndex, :) = acqRes1;
            else
                results(frqBinIndex, :) = acqRes2;
            end
        end
        %% Find Correlation Peaks
        [~, frequencyBinIndex] = max(max(results, [], 2));
        [peakSize, codePhase] = max(max(results));
        samplesPerCodeChip = round(settings.samplingFreq / settings.codeFreqBasis);
        excludeRangeIndex1 = codePhase - samplesPerCodeChip;
        excludeRangeIndex2 = codePhase + samplesPerCodeChip;
        if excludeRangeIndex1 < 1
            codePhaseRange = excludeRangeIndex2:(samplesPerCode + excludeRangeIndex1);
        elseif excludeRangeIndex2 >= samplesPerCode
            codePhaseRange = (excludeRangeIndex2 - samplesPerCode):excludeRangeIndex1;
        else
            codePhaseRange = [1:excludeRangeIndex1, excludeRangeIndex2:samplesPerCode];
        end
        secondPeakSize = max(results(frequencyBinIndex, codePhaseRange));
        acqResults.peakMetric(PRN) = peakSize / secondPeakSize;
        if (peakSize / secondPeakSize) > settings.acqThreshold
            fprintf('%02d ', PRN);
            caCode = generateCAcode(PRN);
            codeValueIndex = floor((ts * (1:10 * samplesPerCode)) / (1 / settings.codeFreqBasis));
            longCaCode = caCode((rem(codeValueIndex, 1023) + 1));
            xCarrier = signal0DC(codePhase:(codePhase + 10 * samplesPerCode - 1)) .* longCaCode;
            fftNumPts = 8 * (2^(nextpow2(length(xCarrier))));
            fftxc = abs(fft(xCarrier, fftNumPts));
            uniqFftPts = ceil((fftNumPts + 1) / 2);
            [~, fftMaxIndex] = max(fftxc);
            fftFreqBins = (0:uniqFftPts - 1) * settings.samplingFreq / fftNumPts;
            if (fftMaxIndex > uniqFftPts)
                if (rem(fftNumPts, 2) == 0)
                    fftFreqBinsRev = -fftFreqBins((uniqFftPts - 1):-1:2);
                    [~, fftMaxIndex] = max(fftxc((uniqFftPts + 1):length(fftxc)));
                    acqResults.carrFreq(PRN) = -fftFreqBinsRev(fftMaxIndex);
                else
                    fftFreqBinsRev = -fftFreqBins((uniqFftPts):-1:2);
                    [~, fftMaxIndex] = max(fftxc((uniqFftPts + 1):length(fftxc)));
                    acqResults.carrFreq(PRN) = fftFreqBinsRev(fftMaxIndex);
                end
            else
                acqResults.carrFreq(PRN) = (-1)^(settings.fileType - 1) * fftFreqBins(fftMaxIndex);
            end
            if (acqResults.carrFreq(PRN) == 0)
                acqResults.carrFreq(PRN) = 1;
            end
            acqResults.codePhase(PRN) = codePhase;
            if (exist('oldFreq', 'var') && settings.resamplingflag == 1)
                acqResults.codePhase(PRN) = floor((codePhase - 1) / settings.samplingFreq * oldFreq) + 1;
                if (settings.IF >= settings.samplingFreq / 2)
                    IF_temp = settings.samplingFreq - settings.IF;
                    doppler = IF_temp - acqResults.carrFreq(PRN);
                else
                    doppler = acqResults.carrFreq(PRN) - settings.IF;
                end
                acqResults.carrFreq(PRN) = doppler + oldIF;
            end
        else
            fprintf('. ');
        end
    end
    fprintf(')\n');
end