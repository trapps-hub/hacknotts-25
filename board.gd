extends Control

var lucinda
var time = 0
var updateTime = true
var winLabel
var seedField

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	lucinda = get_node("QueensBoard/LucindaGrid")
	seedField = get_parent().get_node("Footer/HBoxContainer/LineEdit")
	
	get_parent()			\
		.get_parent()	\
		.get_node("LeftCol/ButtonContainer/genBoardButton") \
		.pressed.connect(self._regenerate)
		
	get_parent()			\
		.get_parent()	\
		.get_node("LeftCol/ButtonContainer/loadBoardButton") \
		.pressed.connect(self._regenerate_with_seed)
	winLabel = get_node("WinLabel")

func _regenerate_with_seed() -> void:
	winLabel.visible = false
	
	lucinda.regenerate_with_seed(seedField.text)
	time = 0
	updateTime = true
	pass

func _regenerate() -> void:
	winLabel.visible = false
	seedField.text = lucinda.regenerate_with_random_seed()
	time = 0
	updateTime = true
	
func endgame() -> void:
	winLabel.visible = true
	updateTime = false

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if(updateTime):
		time += delta
