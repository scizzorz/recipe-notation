# Recipe Notation

This is a _super_ quick and _super_ dirty proof-of-concept for a recipe
notation. It's inspired by [Cooking for Engineers](http://www.cookingforengineers.com/) tabular recipe notation:

![Tabular recipe notation](http://www.matthewbietz.org/blog/wp-content/uploads/screenshot001.jpg)

But it's different. So. Whatever.

Read anything here at your own risk. This is some _bad_ Rust code. I
deliberately cut almost every possible corner to PoC this quickly.

## Usage

```
$ cat recipe | cargo run | dot -Tsvg > recipe.svg
```

## Example

```
1 Cream: butter (225g, softened),
         sugar (300g, granulated)
2 Beat: #1, sour cream (230g)
3 Mix: #2, eggs (2, large)
4 Mix: #3, vanilla extract (15mL)
5 Toast: pecans (25g, chopped)
6 Process to crumbs: #5
7 Whisk: #6, baking powder (14g), salt (1.5g),
         all-purpose flour (125g, sifted)
8 Fold in: #7, #4
```

### Output

![Output](/recipe.svg)
