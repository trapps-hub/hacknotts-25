extends Panel

var styleBox: StyleBoxFlat = get_theme_stylebox("panel").duplicate()

enum SlotState {BLANK, X, QUEEN}
var state
var isValid = true

var cross_icon
var crown_icon
var invalid_icon

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	state = SlotState.BLANK
	get_node("Button").pressed.connect(self.button_pressed)
	
	cross_icon = get_node("Button/CrossIcon")
	crown_icon = get_node("Button/CrownIcon")
	invalid_icon = get_node("Button/InvalidIcon")
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func button_pressed() -> void:
	match self.state:
		SlotState.BLANK:
			self.state = SlotState.X
			cross_icon.visible = true
			# print("X")
		SlotState.X:
			self.state = SlotState.QUEEN
			cross_icon.visible = false
			crown_icon.visible = true
			# call for check board
			if get_parent().check():
				get_parent().get_parent().get_parent().endgame()
			# print("QUEEN")
		SlotState.QUEEN:
			self.state = SlotState.BLANK
			crown_icon.visible = false
			# call for check board
			if get_parent().check():
				get_parent().get_parent().get_parent().endgame()
			# print("BLANK")
	pass

func disableSlot() -> void:
	get_node("Button").disabled = true
	pass

func resetSlot() -> void:
	get_node("Button").disabled = false
	setValidity(true)
	cross_icon.visible = false
	crown_icon.visible = false
	self.state = SlotState.BLANK
	pass

func isQueen() -> bool:
	return self.state == SlotState.QUEEN

func setValidity(validity: bool) -> void:
	self.isValid = validity
	if self.isValid:
		invalid_icon.visible = false
	else:
		invalid_icon.visible = true
	pass 

func setColour(colour: String) -> void:
	styleBox.set("bg_color", colour)
	add_theme_stylebox_override("panel", styleBox)
	pass
