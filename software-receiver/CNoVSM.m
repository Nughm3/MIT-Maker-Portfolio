function CNoValue = CNoVSM(I_P, Q_P, accTime)
% Calculates the carrier-to-noise ratio (C/N0) using the VSM method
%
% CNoValue = CNoVSM(I_P, Q_P, accTime)
%
%   Inputs:
%       I_P, Q_P    - In-phase and quadrature prompt correlator outputs
%       accTime     - Accumulation time (in seconds)
%   Outputs:
%       CNoValue    - Carrier-to-noise ratio (C/N0) in dB-Hz

% Calculate the power of the prompt correlator outputs
P = I_P.^2 + Q_P.^2;

% Calculate the mean power
meanP = mean(P);

% Calculate the variance of the prompt correlator outputs
varP = var(P);

% Calculate the carrier-to-noise ratio (C/N0)
CNoValue = 10 * log10(meanP^2 / (2 * varP * accTime));
end