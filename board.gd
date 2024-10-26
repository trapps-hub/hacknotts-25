extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	get_parent()			\
		.get_parent()	\
		.get_node("LeftCol/genBoardButton") \
		.pressed.connect(get_node("QueensBoard/LucindaGrid").regenerate)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
