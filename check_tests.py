import json

f = open('tests.json', 'r')
tests = json.load(f)

unique_values_per_key = {}

# Collect all values from all lists
all_values = set()
for first_level_key, second_level_dict in tests.items():
    for second_level_key, value_list in second_level_dict.items():
        all_values.update(value_list)

# Find unique values for each first-level key
for first_level_key, second_level_dict in tests.items():
    unique_values_per_key[first_level_key] = {}
    for second_level_key, value_list in second_level_dict.items():
        other_values = all_values - set(value_list)
        unique_values = set(value_list) - other_values
        unique_values_per_key[first_level_key][second_level_key] = list(unique_values)

print(unique_values_per_key)