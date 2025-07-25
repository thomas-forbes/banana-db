# banana-db

A simple sql-like database written in rust for learning and fun.

## Usage

`banana-db` uses BQL (Banana Query Language) to query the database. Here are some examples:

```
gimme Users; // default limit of 1
gimme Users limit 2;
gimme Users where id==5;
gimme Users where id==5 limit 2;

tables;
new table Users {id: Int, name: String};
delete table Users;

insert {id: 5, name: Thomas, value: 4.2} into Users;
```

## Roadmap

- [ ] Tests 😅
- [ ] Indexes
- [ ] Column contraints (unique, foreign key, etc)
- [ ] Benchmarks
- [ ] Not naive storage implementation
