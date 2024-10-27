extends Label


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var time = get_parent().get_parent().get_node("MidCol/Board").time
	var secs = time
	var minutes = int(floor(secs / 60))
	var show_secs = int(floor(secs)) % 60
	
	self.text = "%02d:%02d" % [ minutes, show_secs ]
