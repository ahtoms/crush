# Packing Boxes

This will be our introduction into Structs, Impl and Generics. Your program is a packing simulator for people who are moving houses. In this instace, your program will have 3 types of boxes
`SmallBox`, `MediumBox` and `LargeBox`, each box has a label (`String`), list of items and can only hold a certain amount of weight.

* `SmallBox` can hold 5kgs
* `MediumBox` can hold 10kgs
* `LargeBox` can hold 20kgs

Your program will have `Item` types that which contain a label (`String`) and weight (`f32`) in kgs. 

For each box type, it should be possible to retrieve the following information
* Current label
* Number of items stored in the box
* Current weight of the box
* Remaining weight of the box

For this instance, we will approach it from a naive perspective:
* Create 3 structs for each box, use literals for the weight limit
* Create an item type with the properties label and weight.
* Test your implementation

Afterwards, consider the amount of code repetition (particularly when calculating remaining weight) we have created and how we can eliminate this. Introduce a `trait` called `Packaging` which will do the following:

* Specify `get_weight`, `get_label`, `get_items_count` and `max_weight` as an *abstract* method
* Write a default method for `remaining_weight` which will use `get_weight` and `max_weight` to calcuate the function.

Could we reduce the repetitive code even more?

As for generics, write a function which will accept any `Packaging` type and output the remaining weight of the box that was passed to it.


