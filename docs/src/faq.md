# Frequently Asked Questions

## General Questions

### Why not use cd + autocomplete?

Autocomplete is great, but sometimes it can get annoying with lots of similar named folders.

```text
cwd/
  folder_ab/ <-- Target
  folder_ba/ 
  folder_ac/
  ab_folder/
```

In this scenario, you'd have to press tab multiple times and see what was autocompleted. With lacy, you can just type `y foab`.

In some cases, autocomplete is still useful. Lacy still offers that autocomplete.

### Why not use Z (zoxide or similar tools)?

These tools are great and a great inspiration for lacy, but the goal of lacy is to have a tool that you can use from the first second and in unknown environments, which e.g. Zoxide doesn't, as it first has to learn your behavior.

## Technical Questions

### Can I use 