
# [7DaysOfCode](https://7daysofcode.io/) - Pokegotchi

  

Challenge Solution using Rust to build a cli game proposed by [Giulia Bordignon](https://www.linkedin.com/in/spacecoding/) on [7daysOfCode](https://7daysofcode.io/matricula/csharp), That allows to adopt up to three pokegotchis and play with them.


- [x] Get Info over [Pokeapi](https://pokeapi.co/)

- [x] Adopt Pokegotchi

- [x] Feed and Play with your Pokegotchis

- [x] Abandon your Pokegotchis :cry:

  

The project features async calls provided by tokio, recursive iterations over pages being called(not optimized) with help of async-recursion, data persistence by sqlx + sqlite, reqwest to communicate with pokeapi and chrono to manage timestamps