import sys
import re

def generate_graphviz(input_string):
  lines = input_string.splitlines()
  inputs = {}
  gates = []

  # Parse inputs
  for line in lines:
    if ":" in line:
      match = re.match(r"(\w+): (\d+)", line)
      if match:
        inputs[match.group(1)] = match.group(2)
    elif "->" in line:
      gates.append(line)

  # Generate Graphviz code
  graphviz_code = "digraph logic_gates {\n"

#   # Add input nodes
#   for name, value in inputs.items():
#     graphviz_code += f'  {name} [label="{name}"];\n'

  # Add input nodes and connect pairs with invisible edges
  for name in inputs:
    graphviz_code += f'  {name} [label="{name}"];\n'
    if name.startswith('x'):
      y_name = 'y' + name[1:]  # Get corresponding y node name
      graphviz_code += f'  {name} -> {y_name} [style=invis];\n' 
    
  # Add gates with shapes and connections
  for gate in gates:
    parts = gate.split()
    in1, op, in2, arrow, out = parts
    if op == "OR":
      op = "sOR"

    graphviz_code += f'  {out} [shape=box, label="{op}\n{out}"];\n'
    graphviz_code += f"  {in1} -> {out};\n"
    graphviz_code += f"  {in2} -> {out};\n"

  graphviz_code += "}\n"

  return graphviz_code

input_string = sys.stdin.read()

graphviz_code = generate_graphviz(input_string)
print(graphviz_code)
