
-- Insertar grupo
INSERT INTO grupos (id_yappy, nombre, api_key, secret_key)
VALUES ('kioskos-autoservicio', 'Kiosko UTP', '56377644-baee-4e4d-ac0d-009c55264c3e', 'WVBfMzc4NDE2RTUtMkRFMS0zMTE0LUE1QjgtNzUyQzA4QThCNDE3');

-- Suponiendo que ese grupo se insertó con ID 1

-- Insertar 6 cajas del grupo (tipo kiosko)
INSERT INTO cajas (id_grupo, nombre_caja, tipo, token_autorizacion, estado)
VALUES
(1, 'orillac', 'kiosko', NULL, 'cerrado'),
(1, 'edificio-aulas', 'kiosko', NULL, 'cerrado'),
(1, 'desarrollo-dtwm', 'kiosko', NULL, 'cerrado'),
(1, 'edificio-1', 'kiosko', NULL, 'cerrado'),
(1, 'edificio-3', 'kiosko', NULL, 'cerrado'),
(1, 'centro-regional-veraguas', 'kiosko', NULL, 'cerrado');

-- Ahora insertamos 6 kioskos vinculados con sus respectivas cajas
-- Suponiendo que las cajas quedaron con IDs del 1 al 6

INSERT INTO kioskos (id_caja, nombre, mac_address)
VALUES
(1, 'Edificio de Orillac', 'cc:28:aa:d1:62:66'),
(2, 'Edificio de Aulas', '58:11:22:48:ce:2e'),
(3, 'Desarrollo DTWM', '64:4e:d7:6a:3f:fc'),
(4, 'Edificio 1', 'cc:28:aa:d1:62:8e'),
(5, 'Edificio 3', '58:11:22:48:ce:9c'),
(6, 'Centro Regional de Veraguas', '58:11:22:48:ce:df');
