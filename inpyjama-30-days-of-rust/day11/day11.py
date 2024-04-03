# Python code
treasure_map = ["gold", "diamonds", "ruby"]
another_map = treasure_map

# Modifying another_map changes the original! (Shallow Copy)
another_map[0] = "shiny new sword"

print(treasure_map)  # Output: ["shiny new sword", "diamonds", "ruby"]
