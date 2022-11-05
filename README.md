# Sample It

An interface for managing collections of labeled items and generating random subsets with specified restrictions.

This project was developed as an assignment for the Software Engineering class @dcc/ufmg 2022-2.

## Table of Contents
1. [Building and running](#building-and-running)
2. [Developers](#developers)
3. [Review](#review)
4. [Sprint Backlog](#sprint-backlog)
5. [Product Backlog](#product-backlog)

## Building and running

### Dependencies

- Backend: [Rust](https://www.rust-lang.org/);
- [SQLite](https://www.sqlite.org/index.html): Relational database;
- [CBC](https://github.com/coin-or/Cbc): Open-Source solver for linear programs;
- Frontend: [nodejs and npm](https://nodejs.org/en/).

### Back-end Server

Just execute `cargo run`. The server will be running at `localhost:3000`.

### Front-end Server

The frontend is located in the folder `webapp`, all the commands should be ran inside this folder.
To install the webapp's dependencies, run `npm install`. Then, execute `npm run dev`. The website will be running at `localhost:5173`.

## Developers

- Natan Ventura - Frontend
- Gabriel Fialho - Frontend
- Kaio Vieira - Backend
- Mariano Fernandes - Backend

## Review

The tasks below were suggested at the start of the sprint. The first and third user story were considered completed at the end. We decided not to tackle the second user story, as it wasn't very essential to a minimum viable product and would consume too much time to implement.

## Sprint Backlog

- Technical Tasks:
	- Gain familiarity with the technologies being used. [Gabriel, Kaio, Mariano, Natan]
	- Design the app's stylesheets [Gabriel, Natan].

- User Story: As an user, I would like to manage collections of items using the website.
	- Tasks:
		- Create the web pages. [Gabriel, Natan]
		- Create the database schema. [Mariano]
		- Implement functions to communicate with the database. [Kaio, Mariano]
		- Create the application routes. [Kaio, Mariano]
	- Done Criteria:
		- Being able to easily view, create, update and delete the objects in the website.

- User Story: As an user, I would like to use files to import and export collections.
	- Tasks:
		- Investigate how to get user input from the interface. [Gabriel, Natan]
		- Update the pages to include import/export. [Gabriel, Natan]
		- Define the format of the files. [Kaio, Mariano]
		- Implement the conversion from the files, and database update. [Kaio, Mariano]
	- Done Criteria:
		- Usability of the file format is intuitive and easy.
		- Integration between file input and interface input works without flaws.

- User Story: As an user, I would like to generate random subsets from a given collection, specifying restrictions.
	- Tasks:
		- Create the webpage to generate subsets. [Gabriel, Natan]
		- Implement the business logic of the random sampling. [Kaio, Mariano]
	- Done Criteria:
		- Generate a sample including many restrictions.

## Product Backlog

- User Story: As an user, I would like to choose the random seed to be used in the sampling.
- User Story: As an user, I would like to share the results of a sampling. 
- User Story: As an user, I would like to the labels to have different colors depending on the name.
