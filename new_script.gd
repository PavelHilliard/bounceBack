extends Node2D

var state = "ready"
var start_time = 0
var round = 0

func _ready():
	$Packet.position.x = 275
	$Label.text = "Press SPACE to send a ping"

func _input(event):
	if not event.is_action_pressed("ui_accept"):
		return

	if state == "ready":
		state = "waiting"
		round += 1
		var my_round = round
		$Packet.position.x = 591
		$Label.text = "Ping sent to server... wait for it"
		await get_tree().create_timer(randf_range(1, 3)).timeout
		if state == "waiting" and my_round == round:
			state = "reply"
			$Packet.position.x = 271
			$Label.text = "REPLY! Press SPACE!"
			start_time = Time.get_ticks_msec()

	elif state == "waiting":
		state = "ready"
		$Packet.position.x = 271
		$Label.text = "Too early! Packet dropped. Press SPACE to try again"

	elif state == "reply":
		var ms = Time.get_ticks_msec() - start_time
		state = "ready"
		$Label.text = "Reply received in " + str(ms) + " ms Press SPACE to ping again"
