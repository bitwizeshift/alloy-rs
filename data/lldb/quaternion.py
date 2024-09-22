import lldb

class Quaternion:
  def __init__(self, valobj, internal_dict):
    self.valobj = valobj
    self.update()

  def update(self):
    vec = self.valobj.GetChildMemberWithName('0')
    self.w = vec.GetChildMemberWithName('x')
    self.i = vec.GetChildMemberWithName('y')
    self.j = vec.GetChildMemberWithName('z')
    self.k = vec.GetChildMemberWithName('w')

  def num_children(self):
    return 4

  def get_child_at_index(self, index):
    if index < 0 or index >= 4:
      return None

    try:
      if index == 0:
        return self.w.CreateValueFromData("w", self.w.GetData(), self.w.GetType())
      if index == 1:
        return self.i.CreateValueFromData("i", self.i.GetData(), self.i.GetType())
      if index == 2:
        return self.j.CreateValueFromData("j", self.j.GetData(), self.j.GetType())
      if index == 3:
        return self.k.CreateValueFromData("k", self.k.GetData(), self.k.GetType())
      return None
    except:
      return None

  def get_child_index(self, name):
    if name == "w":
      return 0
    elif name == "i":
      return 1
    elif name == "j":
      return 2
    elif name == "k":
      return 3
    else:
      return -1

def quaternion_summary(valobj, internal_dict):
  try:
    real_valobj = valobj.GetNonSyntheticValue()
    q = Quaternion(real_valobj, internal_dict)

    w = (float(q.w.GetValue()), "")
    i = (float(q.i.GetValue()), "i")
    j = (float(q.j.GetValue()), "j")
    k = (float(q.k.GetValue()), "k")
    result = ""
    for v, name in [w, i, j, k]:
      if v == 0:
        continue
      space = ""
      if result != "":
        space = " "

      if v < 0:
        result += f"{space}-"
      elif result != "":
        result += f"{space}+"

      result += f"{space}{abs(v)}{name}"

    if result == "":
      return "0"

    return result
  except:
    return "Quaternion(err)"

def __lldb_init_module(debugger, internal_dict):
  # Register the pretty printer for Vec4
  debugger.HandleCommand('type synthetic add -l quaternion.Quaternion -x "alloy::math::quaternion::Quaternion"')
  debugger.HandleCommand('type summary add -F quaternion.quaternion_summary -x "alloy::math::quaternion::Quaternion"')
