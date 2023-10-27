# Alpakas

Mini-language built in Rust for the Formal Languages & Compiler Design university course.

**Student: Iulia-Diana Groza, Gr. 933/1**

### Remarks for Lab Assignments:
**1. [Lab 1b] Code review performed for student Daniel Gulei** <br>
&nbsp;&nbsp;&nbsp;&nbsp;Link: https://github.com/trueNebula/PyPlusPlus/pull/1?fbclid=IwAR2TAynQeLoZyZRlwd6L9yvMVrJ48ed4Zmg-B81SUqtnpY-8sGwBtzJl8t4

## 1. Symbol Table
The symbol table uses a single hash table to store both identifiers and constants.
### 1.1 Structure - Hash Table
The hash table is implemented using a fixed-size table with separate chaining to resolve collisions. Each entry in the table is an option containing a vector of entries (representing the chain). Each entry consists of a key-value pair.
### 1.2 Operations
Constructor: Initialises an empty hash table of size TABLE_SIZE.
Hash function: A basic hash function that computes the hash index for a given key.
Insert: Inserts a key-value pair into the hash table.
Get: Retrieves the value associated with a given key.

