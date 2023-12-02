Feature: Snow Island Cube Game
  In order to play the cube game on Snow Island
  As a player
  I want to determine which games are possible given a specific number of cubes

  Scenario Outline: Determine the possible games with a limited number of cubes
    Given a bag is loaded with "<red_cubes>" red cubes, "<green_cubes>" green cubes, and "<blue_cubes>" blue cubes
    And the record of games played with their respective cube reveals
      | game_id | cube_reveals                                 |
      | <game_id> | <cube_reveals>                             |
    When I analyze the games
    Then the possible games should be "<possible_games>"
    And the sum of the IDs of those games should be "<sum_of_ids>"

    Examples:
      | red_cubes | green_cubes | blue_cubes | game_id | cube_reveals                                       | possible_games | sum_of_ids |
      | 12        | 13          | 14         | 1       | 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green     | 1, 2, 5        | 8          |
      | 12        | 13          | 14         | 2       | 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue | 1, 2, 5        | 8          |
      | 12        | 13          | 14         | 3       | 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red | 1, 2, 5        | 8          |
      | 12        | 13          | 14         | 4       | 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red | 1, 2, 5        | 8          |
      | 12        | 13          | 14         | 5       | 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green     | 1, 2, 5        | 8          |

