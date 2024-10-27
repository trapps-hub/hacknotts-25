extends Control

var lucinda
var time = 0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	lucinda = get_node("QueensBoard/LucindaGrid")
	get_parent()			\
		.get_parent()	\
		.get_node("LeftCol/genBoardButton") \
		.pressed.connect(self._regenerate)

func _regenerate() -> void:
	lucinda.regenerate()
	time = 0

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	time += delta
