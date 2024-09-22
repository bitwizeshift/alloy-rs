import lldb

class Vec2:
  def __init__(self, valobj, internal_dict):
    self.valobj = valobj
    self.update()

  def update(self):
    self.data_ptr = self.valobj.GetChildMemberWithName('data_ptr').GetValueAsUnsigned()
    self.length = self.valobj.GetChildMemberWithName('length').GetValueAsUnsigned()

  def num_children(self):
    return self.length

  def get_child_at_index(self, index):
    if index < 0 or index >= 3:
      return None
    if self.data_ptr == 0:
      return None

    try:

      offset = index * 4
      target_type = self.valobj.GetTarget().FindFirstType("float")
      ptr = self.data_ptr + offset
      if index == 0:
        return self.valobj.CreateValueFromAddress("x", ptr, target_type)
      if index == 1:
        return self.valobj.CreateValueFromAddress("y", ptr, target_type)
      return None
    except:
      return None

  def get_child_index(self, name):
    if name == "x":
      return 0
    elif name == "y":
      return 1
    elif name == "z":
      return 2
    else:
      return -1

def vec2_summary(valobj, internal_dict):
  try:
    real_valobj = valobj.GetNonSyntheticValue()
    vec = Vec2(real_valobj, internal_dict)

    if vec.data_ptr == 0:
      return "Vec2(null)"
    if vec.length != 2:
      return "Vec2(?)"

    target_type = real_valobj.GetTarget().FindFirstType("float")

    ptr = vec.data_ptr
    x = real_valobj.CreateValueFromAddress("x", ptr + 0, target_type).GetValue()
    y = real_valobj.CreateValueFromAddress("y", ptr + 4, target_type).GetValue()

    return f"{{x:{x}, y:{y}}}"
  except:
    return "Vec2(err)"

def __lldb_init_module(debugger, internal_dict):
  # Register the pretty printer for Vec2
  debugger.HandleCommand('type synthetic add -l vec2.Vec2 -x "alloy::math::vec::vec2::Vec2"')
  debugger.HandleCommand('type summary add -F vec2.vec2_summary -x "alloy::math::vec::vec2::Vec2"')
