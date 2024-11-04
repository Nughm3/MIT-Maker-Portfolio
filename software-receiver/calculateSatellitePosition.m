function [satPos, satVel] = calculateSatellitePosition(keplerianElements, t)
    % Calculate satellite position and velocity from Keplerian elements
    %
    % [satPos, satVel] = calculateSatellitePosition(keplerianElements, t)
    %
    % Inputs:
    %   keplerianElements - Struct with Keplerian elements
    %   t - Time since epoch [s]
    %
    % Outputs:
    %   satPos - Satellite position [x, y, z] in ECEF coordinates [m]
    %   satVel - Satellite velocity [vx, vy, vz] in ECEF coordinates [m/s]

    mu = 3.986004418e14; % Earth's gravitational parameter [m^3/s^2]

    % Extract Keplerian elements
    a = keplerianElements.a; % Semi-major axis [m]
    e = keplerianElements.e; % Eccentricity
    i = keplerianElements.i; % Inclination [rad]
    omega = keplerianElements.omega; % Argument of perigee [rad]
    Omega = keplerianElements.Omega; % Right ascension of ascending node [rad]
    M0 = keplerianElements.M0; % Mean anomaly at epoch [rad]
    n = sqrt(mu / a^3); % Mean motion [rad/s]
    
    % Mean anomaly at time t
    M = M0 + n * t;

    % Solve Kepler's equation for eccentric anomaly E
    E = M;
    for k = 1:10
        E = M + e * sin(E);
    end

    % True anomaly
    nu = 2 * atan(sqrt((1 + e) / (1 - e)) * tan(E / 2));

    % Distance
    r = a * (1 - e * cos(E));

    % Position in orbital plane
    x_orb = r * cos(nu);
    y_orb = r * sin(nu);
    z_orb = 0;

    % Velocity in orbital plane
    vx_orb = -sqrt(mu / a) * sin(E);
    vy_orb = sqrt(mu / a) * sqrt(1 - e^2) * cos(E);
    vz_orb = 0;

    % Rotation matrices
    R1 = [cos(Omega), -sin(Omega), 0;
          sin(Omega), cos(Omega), 0;
          0, 0, 1];
    R2 = [1, 0, 0;
          0, cos(i), -sin(i);
          0, sin(i), cos(i)];
    R3 = [cos(omega), -sin(omega), 0;
          sin(omega), cos(omega), 0;
          0, 0, 1];

    % Position and velocity in ECEF
    R = R1 * R2 * R3;
    satPos = R * [x_orb; y_orb; z_orb];
    satVel = R * [vx_orb; vy_orb; vz_orb];
end