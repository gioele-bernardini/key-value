#!/usr/bin/env python3

# (len / height) = ratio;
# hence => height = (len / ratio);
DDRAM4_RATIO = 4.5

def print_header(num_columns):
  header = "| "
  for i in range(num_columns):
    header += "   |"

  print(header)


  header = "| "
  for i in range(num_columns):
    header += " - |"

  print(header)

def print_table(num_rows, num_columns):
  total_elements = num_rows * num_columns
  for i in range(0, total_elements, num_columns):
    row = "| "
    for j in range(num_columns):
      number = i + j
      if number < 10:
        row += f"0{number} | "
      else:
        row += f"{number} | "

    print(row)

if __name__ == '__main__':
  num_columns = 15
  num_rows = int (num_columns / DDRAM4_RATIO)

  print_header(num_columns)
  print_table(num_rows, num_columns)
