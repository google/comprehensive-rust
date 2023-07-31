import csv

def main():
  lines = csv.reader(open("outline.csv"))
  next(lines)
  print("[")
  for line in lines:
    if line[0] == '':
      continue
    if line[2] in ('Break', 'Lunch', '', 'End of day'):
      continue
    if line[0].startswith('Rust'):
      break
    del line[0]
    (minutes, title, existing, notes) = line
    minutes = int(minutes) if minutes else None
    print(f"  Slide({title=},\n    {existing=},\n    {minutes=},\n    {notes=}),")
  print("]")

main()
