package main

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/gotk3/gotk3/gtk"
)

func main() {
	// Initialize GTK
	gtk.Init(nil)

	// Create a new window
	win, err := gtk.WindowNew(gtk.WINDOW_TOPLEVEL)
	if err != nil {
		panic(err)
	}
	win.SetTitle("Guess the Number")
	win.Connect("destroy", func() {
		gtk.MainQuit()
	})

	// Create a vertical box to hold the widgets
	vbox, err := gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 5)
	if err != nil {
		panic(err)
	}
	win.Add(vbox)

	// Create a label to display the instructions
	instructions, err := gtk.LabelNew("I'm thinking of a number between 1 and 100. Can you guess what it is?")
	if err != nil {
		panic(err)
	}
	vbox.PackStart(instructions, false, false, 0)

	// Create an entry widget for the player to enter their guess
	entry, err := gtk.EntryNew()
	if err != nil {
		panic(err)
	}
	vbox.PackStart(entry, false, false, 0)

	// Create a label to display the feedback
	feedback, err := gtk.LabelNew("")
	if err != nil {
		panic(err)
	}
	vbox.PackStart(feedback, false, false, 0)

	// Seed the random number generator
	rand.Seed(time.Now().UnixNano())

	// Generate a random number between 1 and 100
	secretNumber := rand.Intn(100) + 1

	// Connect the "activate" signal of the entry widget to a function
	entry.Connect("activate", func() {
		// Get the text from the entry widget
		text, err := entry.GetText()
		if err != nil {
			panic(err)
		}

		// Convert the text to an integer
		guess, err := strconv.Atoi(text)
		if err != nil {
			feedback.SetText("Please enter a valid number.")
			return
		}

		// Give the player feedback on their guess
		if guess < secretNumber {
			feedback.SetText("Your guess is too low. Try again.")
		} else if guess > secretNumber {
			feedback.SetText("Your guess is too high. Try again.")
		} else {
			feedback.SetText("Congratulations! You
