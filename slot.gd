extends Panel

var styleBox: StyleBoxFlat = get_theme_stylebox("panel").duplicate()

enum SlotState {BLANK, X, QUEEN}
var state
var isValid = true

var cross_icon
var crown_icon

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	state = SlotState.BLANK
	get_node("Button").pressed.connect(self.button_pressed)
	
	cross_icon = get_node("Button/CrossIcon")
	crown_icon = get_node("Button/CrownIcon")
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func button_pressed() -> void:
	match self.state:
		SlotState.BLANK:
			self.state = SlotState.X
			cross_icon.visible = true
			print("X")
		SlotState.X:
			self.state = SlotState.QUEEN
			cross_icon.visible = false
			crown_icon.visible = true
			print("QUEEN")
		SlotState.QUEEN:
			self.state = SlotState.BLANK
			crown_icon.visible = false
			print("BLANK")
	pass

func resetSlot() -> void:
	isValid = true
	cross_icon.visible = false
	crown_icon.visible = false
	self.state = SlotState.BLANK
	pass

func setColour(colour: String) -> void:
	styleBox.set("bg_color", colour)
	add_theme_stylebox_override("panel", styleBox)
	pass
