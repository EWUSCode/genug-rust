# Genug Rust für den Anfang

Dieses Repository ist das Begleitmaterial zur [Playlist »Genug Rust für den Anfang«](https://www.youtube.com/playlist?list=PLiVx-ZPFT5CgChCpvCM1IhqcmsdJnIy1G).

Es werden diese Themen behandelt:

1. [Umgebung einrichten](https://youtu.be/-j4gVscGmHU)
2. [Konfiguration und Logging](https://youtu.be/Bibap2SZtco)
3. [HTTP](https://youtu.be/R4PSjFBa5LI)
4. [Datenübergabe](https://youtu.be/p091TDADYO0)
5. [Datenbank](https://youtu.be/U--W0ZEkSmQ)
6. [Authentifizierung](https://youtu.be/fAgMUk6A1o4)

Diskussion und Kommentare können gern im [Blog](https://ewus.de/blog/2023-10-21/genug-rust-fuer-den-anfang) erfolgen.

## Entwicklung

Um die laufende Entwicklungsumgebung zu betreten, kann dieser Befehl genutzt werden:

```bash
docker compose -f .devcontainer/docker-compose.yml exec --user ${UID} app /bin/bash
```

## Datenbank

Die Datenbank (Oracle Container) wird mit folgenden Befehlen eingerichtet:

```bash
docker compose -f .devcontainer/docker-compose.yml exec ora /bin/bash
./setPassword.sh gjx4CRPL
sqlplus sys/gjx4CRPL@localhost:1521/FREEPDB1 as sysdba
CREATE USER smith IDENTIFIED BY S94dDMHs;
GRANT CREATE SESSION TO smith;
exit;
exit
```
