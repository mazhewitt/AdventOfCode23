Feature: Snow Island Cube Game
  In order to play the cube game on Snow Island
  As a player
  I want to determine which games are possible given a specific number of cubes

  Scenario: Determine the possible games with a limited number of cubes
    Given a bag with 12 red cubes, 13 green cubes, 14 blue cubes
    When Game 1: 3 blue, 4 red, 1 red, 2 green, 6 blue, 2 green
    And Game 2: 1 blue, 2 green, 3 green, 4 blue, 1 red, 1 green, 1 blue
    And Game 3: 8 green, 6 blue, 20 red, 5 blue, 4 red, 13 green, 5 green, 1 red
    And Game 4: 1 green, 3 red, 6 blue, 3 green, 6 red, 3 green, 15 blue, 14 red
    And Game 5: 6 red, 1 blue, 3 green, 2 blue, 1 red, 2 green
    Then the possible games are "1, 2, 5" and the sum is 8

