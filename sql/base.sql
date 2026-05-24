CREATE TABLE Artistas (
    id_artista SERIAL PRIMARY KEY,
    nombre_artistico VARCHAR(100) NOT NULL,
    genero_principal VARCHAR(50)
);

CREATE TABLE Albumes (
    id_album SERIAL PRIMARY KEY,
    titulo VARCHAR(100) NOT NULL,
    fecha_lanzamiento DATE,
    id_artista INT REFERENCES Artistas(id_artista)
);

CREATE TABLE Canciones (
    id_cancion SERIAL PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    duracion TIME, -- Formato HH:MM:SS
    id_album INT REFERENCES Albumes(id_album)
);

CREATE TABLE Usuarios_Streaming (
    id_usuario SERIAL PRIMARY KEY,
    nombre_usuario VARCHAR(50) UNIQUE NOT NULL,
    tipo_suscripcion VARCHAR(20) CHECK (tipo_suscripcion IN ('Free', 'Premium'))
);

CREATE TABLE Playlists (
    id_playlist SERIAL PRIMARY KEY,
    nombre_lista VARCHAR(100) NOT NULL,
    id_usuario INT REFERENCES Usuarios_Streaming(id_usuario),
    fecha_creacion DATE DEFAULT CURRENT_DATE
);