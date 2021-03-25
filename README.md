## Pud

Human friendly calculator app for Linux

**This is a working in progress project, there are a lot of things to be done yet.**

### Introduction

Pud is a calculator app for Linux that tries to emulate natural language for describing
the equations, it also allows taking notes with the math. With Pud, you can write things
like this:

```
This delicious ice cream costs 5.0 plus 0.25 after taxes // Pud evaluates to 5.25
```

### Grammar

Pud has a special grammar so math and text can come together.

Everything that can't be parsed into an expression is a text, so there are no reserved
keywords, here are some expressions you can write with Pud:

#### Basic Arithmetic

- Addition:
    - `2 + 2`
    - `5 plus 10`
    - `6 with 4`
- Subtraction:   
    - `2 - 2`
    - `5 minus 10`
    - `6 subtract 4`
- Multiplication:
    - `2 * 2`
    - `5 mul 10`
    - `6 times 4`
    - `3 multiplied by 6`
- Division:
    - `3 / 2`
    - `10 div 5`
    - `6 divide 4`
    - `7 divided by 7`
- Power
    - ` 2 ^ 2`
    - ` 3 ** 4`
 
 #### Floating Point
 
 Pud supports single precision floating-point math, so you can calculate `2.3456 + 3.4`,
 if you omit the decimals Pud will do his best to evaluate the answer to `integer`.
 For example:
 
 - `2 + 1`: answer is an integer because both terms are integer.
 - `2.5 + 1`: answer is a float because one of the terms is float.
 - `5 / 3` or `2 ** 5`: always evaluates to float for the sake of simplicity.
 
### TO-DO

- [x] Graphical User Interface
- [ ] Speed, Distance, Temperature and Time conversions.
- [ ] Advanced math functions: `ln`, `exp`, etc.
- [ ] Currencies and Cryptocurrencies conversions.
- [ ] `.deb` and `.rpm` packages.
 


