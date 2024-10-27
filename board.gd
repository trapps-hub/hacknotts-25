extends Control

var lucinda
var time = 0
var updateTime = true
var winLabel

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	lucinda = get_node("QueensBoard/LucindaGrid")
	get_parent()			\
		.get_parent()	\
		.get_node("LeftCol/genBoardButton") \
		.pressed.connect(self._regenerate)
	winLabel = get_node("WinLabel")

func _regenerate() -> void:
	winLabel.visible = false
	lucinda.regenerate()
	time = 0
	updateTime = true
	
func endgame() -> void:
	winLabel.visible = true
	updateTime = false

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if(updateTime):
		time += delta
