function [tau1, tau2] = calcLoopCoef(loopNoiseBandwidth, loopDampingRatio, loopGain)

% [tau1, tau2] = calcLoopCoef(loopNoiseBandwidth, loopDampingRatio, loopGain)
%
%   Inputs:
%       loopNoiseBandwidth   - Loop noise bandwidth (in Hz)
%       loopDampingRatio     - Loop damping ratio
%       loopGain             - Loop gain
%   Outputs:
%       tau1, tau2           - Loop filter coefficients

omega_n = loopNoiseBandwidth / (sqrt(2) * loopDampingRatio);

tau1 = loopGain / (omega_n^2);
tau2 = 2 * loopDampingRatio / omega_n;
end