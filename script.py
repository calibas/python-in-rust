# script.py

def transform_data(data):
    """
    data is expected to be a list of dicts, e.g.:
    [
      {"id": "item1", "value": 10},
      {"id": "item2", "value": 42},
    ]

    We'll return a new list, adding "result" = value * 2
    """
    new_data = []
    for item in data:
        # Copy to avoid mutating the original
        copied = dict(item)
        # Add a new key
        copied["result"] = copied["value"] * 3
        new_data.append(copied)
    return new_data