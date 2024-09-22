import lldb

class Color:
  def __init__(self, valobj, internal_dict):
    self.valobj = valobj
    self.update()

  def update(self):
    vec = self.valobj.GetChildMemberWithName('0')
    self.r = vec.GetChildMemberWithName('x')
    self.g = vec.GetChildMemberWithName('y')
    self.b = vec.GetChildMemberWithName('z')
    self.a = vec.GetChildMemberWithName('w')

  def num_children(self):
    return 4

  def get_child_at_index(self, index):
    if index < 0 or index >= 4:
      return None

    try:
      if index == 0:
        return self.r.CreateValueFromData("r", self.r.GetData(), self.r.GetType())
      if index == 1:
        return self.g.CreateValueFromData("g", self.g.GetData(), self.g.GetType())
      if index == 2:
        return self.b.CreateValueFromData("b", self.b.GetData(), self.b.GetType())
      if index == 3:
        return self.a.CreateValueFromData("a", self.a.GetData(), self.a.GetType())
      return None
    except:
      return None

  def get_child_index(self, name):
    if name == "r":
      return 0
    elif name == "g":
      return 1
    elif name == "b":
      return 2
    elif name == "a":
      return 3
    else:
      return -1

def color_summary(valobj, internal_dict):
  try:
    real_valobj = valobj.GetNonSyntheticValue()
    color = Color(real_valobj, internal_dict)

    r = int(float(color.r.GetValue()) * 255)
    g = int(float(color.g.GetValue()) * 255)
    b = int(float(color.b.GetValue()) * 255)
    a = int(float(color.a.GetValue()) * 255)
    return "#{:02X}{:02X}{:02X}{:02X}".format(r, g, b, a)
  except:
    return "Color(...)"

def __lldb_init_module(debugger, internal_dict):
  debugger.HandleCommand('type synthetic add -l color.Color -x "alloy::model::color::Color"')
  debugger.HandleCommand('type summary add -F color.color_summary -x "alloy::model::color::Color"')
