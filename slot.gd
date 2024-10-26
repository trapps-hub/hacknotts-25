extends Panel

var styleBox: StyleBoxFlat = get_theme_stylebox("panel").duplicate()

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func setColour(colour: String) -> void:
	styleBox.set("bg_color", colour)
	add_theme_stylebox_override("panel", styleBox)
	pass
