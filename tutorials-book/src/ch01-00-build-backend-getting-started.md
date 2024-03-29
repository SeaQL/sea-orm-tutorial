# Chapter 1 - Building a Backend with SeaORM

_Full source code available on [GitHub](https://github.com/SeaQL/sea-orm-tutorial/tree/master/bakery-backend)._

In this chapter, we will build a backend application with SeaORM. It will act as a layer of communication with the database.

The application will simulate the interface of a database of bakeries. For simplicity, there will be only two entities, `Bakery` and `Chef`. We will explore the schema later on.

## Choosing a Database

Before we start building the backend, we want to make sure that the database is up and running. Setting up the database is beyond the scope of this tutorial.

SeaORM itself is agnostic to different database implementations, including MySQL, PostgreSQL, and SQLite (in files or in memory).

However, depending on the database of your choice, you need to pay attention to the following:

-   The appropriate [DB driver feature](https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime#database_driver) should be enabled.
-   A valid database URL should be used:

|      Database      |         Example Database URL          |
| :----------------: | :-----------------------------------: |
|       MySQL        |  `mysql://root:root@localhost:3306`   |
|     PostgreSQL     | `postgres://root:root@localhost:5432` |
|  SQLite (in file)  |     `sqlite:./sqlite.db?mode=rwc`     |
| SQLite (in memory) |           `sqlite::memory:`           |

We will showcase exactly how to and where to use them in the next section.

Once the database is ready, we can proceed to [set up the project](ch01-01-project-setup.md).
