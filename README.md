# Sample It

An interface for managing collections of labeled items and generating random subsets with specified restrictions.

## User Stories

- As a maths teacher, I want to select random questions from a database, specifying that I want $2$ easy, $3$ medium and $1$ hard question in the exam. Furthermore, I could also set the number of trigonometry questions to be at least $1$ and at most $2$. And so on.

- As a playlist maker, I would like to select some musics for a playlist with $15$ songs. From those musics, I want at least $3$, and at most $5$ pop songs; no more than one sad song; at least $4$ electronic songs; and so on.

## Functionality

1. List, Create, Update and Delete: Collections, Items and Labels

As an user, I would like to manage my collections, items and labels. I would like to be able to import and export them to the interface; and be able to quickly access recently used collections.

2. Random Sampling

As an user, I would like to generate random subsets from a given collection. In doing so, I want to specify the number of items to be selected, and add restrictions of the type “At least $x$ and at most $y$ items with this label.” It would be nice to be able to specify the random seed and have the option of removing the items from the database.

[Here](https://www.figma.com/file/6aL6cJVWIaWudIQTumFRf1/Sample-It?node-id=0%3A1) you can find sketches for the page designs.

## Developers

- Natan Ventura - Frontend
- Gabriel Fialho - Frontend
- Kaio Vieira - Backend
- Mariano Fernandes - Backend

## Technologies

- Backend: [Rust](https://www.rust-lang.org/), using the [actix framework](https://github.com/actix/actix-web);
- [SQLite](https://www.sqlite.org/index.html): Relational database;
- [CBC](https://github.com/coin-or/Cbc): Open-Source solver for linear programs;
- Frontend: [Vuejs](https://vuejs.org/).

This project is being developed as an assignment for the Software Engineering class @dcc/ufmg.
