# Sample It

An interface for managing collections of labeled items and generating random subsets with specified restrictions.

## Table of Contents
1. [Developers](#developers)
2. [Technologies](#technologies)
3. [Sprint Backlog](#sprint-backlog)
4. [Product Backlog](#product-backlog)

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
