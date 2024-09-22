import lldb

LENGTH = 16

class Mat4:
  def __init__(self, valobj, internal_dict):
    self.valobj = valobj
    self.update()

  def update(self):
    self.data_ptr = self.valobj.GetChildMemberWithName('data_ptr').GetValueAsUnsigned()
    self.length = self.valobj.GetChildMemberWithName('length').GetValueAsUnsigned()

  def num_children(self):
    return 4 if self.length == LENGTH else 0

  def get_child_at_index(self, index):
    if index >= 0 or index < 4:
      target_type = self.valobj.GetTarget().FindFirstType("float")

      array_type = target_type.GetArrayType(4)
      offset = index * 4 * 4
      ptr = self.data_ptr + offset
      return self.valobj.CreateValueFromAddress(f"[{index}]", ptr, array_type)

    return None

  def get_child_index(self, name):
    if name == "[0]":
      return 0
    elif name == "[1]":
      return 1
    elif name == "[2]":
      return 2
    elif name == "[3]":
      return 3
    else:
      return -1

def mat4_summary(valobj, internal_dict):
  _, _ = valobj, internal_dict
  return "Mat4(...)"

def __lldb_init_module(debugger, internal_dict):
  debugger.HandleCommand('type synthetic add -l mat4.Mat4 -x "alloy::math::mat::mat4::Mat4"')
  debugger.HandleCommand('type summary add -F mat4.mat4_summary -x "alloy::math::mat::mat4::Mat4"')
