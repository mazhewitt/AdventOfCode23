Feature: Snow Island Cube Game
  In order to play the cube game on Snow Island
  As a player
  I want to determine which games are possible given a specific number of cubes

  Scenario: Determine the possible games with a limited number of cubes
    Given a bag with 12 red cubes, 13 green cubes, 14 blue cubes
    When game 1 reveals 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Then the possible games are "1, 2, 5" and the sum is 8

