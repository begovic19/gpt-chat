import pygame

# Initialize Pygame
pygame.init()

# Set the window size
window_size = (800, 600)
window = pygame.display.set_mode(window_size)

# Set the window title
pygame.display.set_caption("My Game Client")

# Set the background color
bg_color = (255, 255, 255)

# Main game loop
running = True
while running:
    # Handle events
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    # Update game logic here...

    # Draw the game
    window.fill(bg_color)
    # Draw game objects here...
    pygame.display.flip()

# Clean up
pygame.quit()
