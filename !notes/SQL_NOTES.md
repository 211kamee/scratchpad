# SQL [RDBMS] Notes

## 1. Query Running Order

`FROM + JOIN` → `WHERE` → `GROUP BY` → `HAVING` → `SELECT` → `ORDER BY` → `LIMIT`

---

## 2. Category Table

| Category | Commands |
|---|---|
| DDL | CREATE, ALTER, DROP, TRUNCATE |
| DML | INSERT, UPDATE, DELETE |
| DQL | SELECT |
| DCL | GRANT, REVOKE |
| TCL | COMMIT, ROLLBACK, SAVEPOINT |
| Keys | PRIMARY, FOREIGN |
| Constraints | NOT NULL, UNIQUE, CHECK, DEFAULT |
| Set ops | UNION, UNION ALL |
| Joins | INNER, LEFT, RIGHT, OUTER, CROSS |
| Conditional | CASE |
| Operators | + − × ÷ = != <> BETWEEN LIKE IN AND OR IS NULL |
| Aggregates | MAX, MIN, AVG, COUNT, SUM |
| Clauses | WHERE, GROUP BY, HAVING, ORDER BY, LIMIT, AS, DISTINCT |

---

## 3. Commands

### DDL – Definition
- `CREATE TABLE t (col1 TYPE, col2 TYPE CONSTRAINT);`
- `ALTER` – rename table/column, change type, add/drop column, add/drop constraint
- `DROP TABLE t;` / `DROP COLUMN c;`

### DML – Manipulation
- `INSERT INTO t (c1,c2,c3) VALUES ('v1',2,3);`
- `UPDATE` – modify data
- `DELETE FROM t WHERE c1='value';`

### DQL – Query
```sql
SELECT col FROM t WHERE cond LIMIT 5;
```

### DCL – Data Control
`GRANT`, `REVOKE`

### TCL – Transaction Control
`COMMIT`, `ROLLBACK`, `SAVEPOINT`

### Keys
- Primary key = NOT NULL + UNIQUE
- Foreign key = primary key of another table

### Combining Data
- `UNION` – add rows (no duplicates)
- `UNION ALL` – add rows (duplicates allowed)
- `JOIN` – Inner, Left, Right, Outer, Cross

### CASE
```sql
SELECT first_name, last_name, age,
CASE WHEN age > 30 THEN 'Old' ELSE 'Young' END
FROM table_xyz;
```

### Operators
- Arithmetic: `+ - × ÷`
- Comparison: `= != <>`
- Logical: `AND OR`
- Others: `BETWEEN LIKE IN`
- Functions: `MAX MIN AVG COUNT`
- Keywords: `AS`, `ORDER BY`, `HAVING`

---

## 4. Sample Tables

**employees**

| id | first_name | age | dept | salary |
|---|---|---|---|---|
| 1 | Ravi | 35 | IT | 60000 |
| 2 | Anna | 28 | HR | 45000 |
| 3 | John | 42 | IT | 80000 |
| 4 | Mia | 25 | HR | 40000 |
| 5 | Sam | 31 | Sales | 50000 |

**departments**

| dept | head |
|---|---|
| IT | Priya |
| HR | Omar |

---

## 5. Queries and Outputs

### SELECT + WHERE + LIMIT
```sql
SELECT first_name, age FROM employees WHERE age > 30 LIMIT 2;
```
| first_name | age |
|---|---|
| Ravi | 35 |
| John | 42 |

### GROUP BY
```sql
SELECT dept, COUNT(*) FROM employees GROUP BY dept;
```
| dept | COUNT(*) |
|---|---|
| IT | 2 |
| HR | 2 |
| Sales | 1 |

### HAVING
```sql
SELECT dept, AVG(salary) FROM employees GROUP BY dept HAVING AVG(salary) > 45000;
```
| dept | AVG(salary) |
|---|---|
| IT | 70000 |
| Sales | 50000 |

### ORDER BY + LIMIT
```sql
SELECT first_name, salary FROM employees ORDER BY salary DESC LIMIT 1;
```
| first_name | salary |
|---|---|
| John | 80000 |

### BETWEEN / IN / LIKE
```sql
SELECT first_name FROM employees WHERE age BETWEEN 25 AND 31;  -- Anna, Mia, Sam
SELECT first_name FROM employees WHERE dept IN ('HR','Sales'); -- Anna, Mia, Sam
SELECT first_name FROM employees WHERE first_name LIKE 'M%';   -- Mia
```

### DISTINCT
```sql
SELECT DISTINCT dept FROM employees;
```
→ IT, HR, Sales

### CASE
```sql
SELECT first_name, CASE WHEN age > 30 THEN 'Old' ELSE 'Young' END AS grp FROM employees;
```
| first_name | grp |
|---|---|
| Ravi | Old |
| Anna | Young |
| John | Old |
| Mia | Young |
| Sam | Old |

### INNER JOIN
```sql
SELECT e.first_name, d.head FROM employees e
INNER JOIN departments d ON e.dept = d.dept;
```
| first_name | head |
|---|---|
| Ravi | Priya |
| Anna | Omar |
| John | Priya |
| Mia | Omar |

(Sam excluded – no matching dept)

### LEFT JOIN
Same as above, plus `Sam | NULL`.

### UNION / UNION ALL
```sql
SELECT dept FROM employees UNION SELECT dept FROM departments;      -- IT, HR, Sales
SELECT dept FROM employees UNION ALL SELECT dept FROM departments; -- 7 rows (duplicates kept)
```

### DML
```sql
INSERT INTO employees (id, first_name, age, dept, salary) VALUES (6,'Zoe',29,'IT',55000); -- 1 row inserted
UPDATE employees SET salary = 65000 WHERE id = 1;  -- 1 row updated
DELETE FROM employees WHERE id = 4;                -- 1 row deleted
```

### DDL
```sql
CREATE TABLE projects (id INT PRIMARY KEY, name VARCHAR(50) NOT NULL);
ALTER TABLE projects ADD budget INT;
DROP TABLE projects;
```
Each returns: `Query OK`

### TCL / DCL
```sql
BEGIN; DELETE FROM employees WHERE id = 5; ROLLBACK;  -- Sam is back
GRANT SELECT ON employees TO user1;
REVOKE SELECT ON employees FROM user1;
```

---

## 6. Transactions

A transaction is a group of SQL statements executed as one unit: all succeed or none do.

```sql
BEGIN;                                              -- start
UPDATE accounts SET balance = balance - 500 WHERE id = 1;
SAVEPOINT sp1;                                      -- checkpoint
UPDATE accounts SET balance = balance + 500 WHERE id = 2;
ROLLBACK TO sp1;                                    -- undo back to checkpoint
COMMIT;                                             -- make permanent
```

| Command | Purpose |
|---|---|
| BEGIN / START TRANSACTION | Start a transaction |
| COMMIT | Save changes permanently |
| ROLLBACK | Undo all changes since BEGIN |
| SAVEPOINT | Set a point to roll back to |
| ROLLBACK TO sp | Undo up to a savepoint |

### Transaction States
Active → Partially Committed → Committed
Active → Failed → Aborted (rolled back)

---

## 7. ACID Properties

| Property | Meaning | Example |
|---|---|---|
| **A**tomicity | All or nothing | Debit and credit both happen, or neither |
| **C**onsistency | DB moves from one valid state to another (constraints hold) | Total money stays the same |
| **I**solation | Concurrent transactions don't interfere | T1 can't see T2's uncommitted data |
| **D**urability | Committed data survives crashes | Saved even after power failure |

### Isolation Levels

| Level | Dirty Read | Non-repeatable Read | Phantom Read |
|---|---|---|---|
| Read Uncommitted | Yes | Yes | Yes |
| Read Committed | No | Yes | Yes |
| Repeatable Read | No | No | Yes |
| Serializable | No | No | No |

- **Dirty read**: reading uncommitted data
- **Non-repeatable read**: same row gives different values within one transaction
- **Phantom read**: same query returns new rows within one transaction

---

## 8. Normalization

Organizing tables to reduce redundancy and avoid anomalies (insert, update, delete).

| Form | Rule |
|---|---|
| **1NF** | Atomic values only; no repeating groups or multi-valued cells |
| **2NF** | 1NF + no partial dependency (non-key columns depend on the whole primary key) |
| **3NF** | 2NF + no transitive dependency (non-key columns depend only on the key) |
| **BCNF** | 3NF + every determinant is a candidate key |

### Example
**Not 1NF**
| id | name | phones |
|---|---|---|
| 1 | Ravi | 111, 222 |

**1NF**: one phone per row
| id | name | phone |
|---|---|---|
| 1 | Ravi | 111 |
| 1 | Ravi | 222 |

**3NF**: `employees(id, name, dept_id)` + `departments(dept_id, dept_name)` instead of repeating `dept_name` in every employee row.

**Denormalization**: deliberately adding redundancy to speed up reads.

---

## 9. Other Key Concepts

| Concept | Meaning |
|---|---|
| Candidate key | Column(s) that can uniquely identify a row |
| Composite key | Primary key made of 2+ columns |
| Super key | Any set of columns that uniquely identifies a row |
| Index | Structure that speeds up lookups (slower writes) |
| View | Saved query acting as a virtual table |
| Stored procedure | Saved block of SQL that can be re-run |
| Trigger | Code that runs automatically on INSERT/UPDATE/DELETE |
| Subquery | Query nested inside another query |
| CTE | Temporary named result: `WITH x AS (...) SELECT ...` |
| Window function | Calculation across related rows: `RANK() OVER (...)` |
| Referential integrity | Foreign key values must exist in the parent table |
| ON DELETE CASCADE | Deleting a parent row deletes its child rows |
| OLTP vs OLAP | Transaction processing vs analytics workloads |
| CAP / BASE | Distributed/NoSQL counterpart to ACID |
