# Changelog

## [0.2.0]  - 2024-01-10

* reworks crate reorganizing modules

* tax_stage is not a module anymore

* `taxable` methods were removed

* `scale` params were removed

* `Calculation` struct changed to contain 2 struct with values with and without discount

## [0.1.10] - 2024-01-05

* Adds field `recalculated_unit_value` to structs `Calculation`, `CalculationF64`, `CalculationString` 
             to store unit value recalculated from brute after round 

## [0.1.8] - 2024-01-05

* Update depes

## [0.1.7] - 2024-01-04

* Adds taxable method to get the value of the taxable used in the tax stage.

* implements fmt::Display trait for CalculationF64

* introduces line_tax methods

* Adds more doc blocks.

## [0.1.6] - 2023-12-31

* Corrects crate's definition

* Updates README.md

## [0.1.5] - 2023-12-30

* Remove unnecessary deps 

## [0.1.4] - 2023-12-30

* Adds more doctests and resolves typos

* Repair tests
