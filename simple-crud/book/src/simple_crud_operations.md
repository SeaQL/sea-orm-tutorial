# Simple CRUD Operations

In this section, we will perform Create, Read, Update and Delete operations on the `fruits` table in the `fruit_markets` database. Each sub-section will focus on one database operation.

### Creating the database configuration

Create a `.env` file that will hold the database configuration and open it in your favourite code editor. Then add the following configuration in the format `DATABASE_URL=database://username:password@localhost/database_name`

**File:** *./SimpleCRUD/.env*

```sh
# Add this line to the .env file
DATABASE_URL=mysql://webmaster:master_char@localhost/fruit_markets
```

