class Point:
    x_value = 0
    y_value = 0

    def __init__(self, x_value, y_value):
        self.x_value = x_value
        self.y_value = y_value


class SplineSpan:
    coefficients = []
    start_control_point = None
    end_control_point = None


class Term:
    coefficient = 0
    degree = 0

    def __init__(self, coefficient, degree):
        self.coefficient = coefficient
        self.degree = degree



def create_entire_spline(control_points: list[Point]):
    """Creates the entire spline"""

    x_control_points = []
    y_control_points = []

    for x in range(len(control_points)):
        x_control_points.append(Point(x, control_points[x].x_value))
        y_control_points.append(Point(x, control_points[x].y_value))

    x_spline_spans = get_spline_spans(x_control_points)
    y_spline_spans = get_spline_spans(y_control_points)


def get_spline_spans(control_points: list[Point], start_velocity, end_velocity):
    """Creates part of the spline (x or y)"""

    # t^3 + t^2 + t^1 + t^0
    function_terms = [Term(1, 3), Term(1, 2), Term(1, 1), Term(1, 0)]
    # 3t^2 + 2t + 1 + 0
    derivative_terms = [Term(3, 2), Term(2, 1), Term(1, 0), Term(0, 0)]
    # 6t + 2t + 0 + 0
    second_derivative_terms = [Term(6, 1), Term(2, 0), Term(0, 0), Term(0, 0)]

    matrix = []
    number_of_spline_spans = len(control_points) - 1

    for x in range(number_of_spline_spans):
        values = get_values(function_terms, x)  # Gets the values of the function at that t_value (AKA x coordinate)
        # Gets the full row of the values for the matrix. The params are: index, values, total_number_of_indexes, expected_value
        # For instance, get_full_row(1, [5, 6, 7, 8], 1, 2) -> [0, 0, 0, 0, 5, 6, 7, 8, 2]
        matrix.append(get_full_row(x, values, number_of_spline_spans, control_points[x].y_value))

        # Now let's find the rest (assuming you know how the functions work already)
        values = get_values(function_terms, x + 1)
        matrix.append(get_full_row(x + 1, values, number_of_spline_spans, control_points[x + 1].y_value))

    for x in range(1, number_of_spline_spans):
        # Finds the full row of values so that values1 = values2 (the rest of the values are 0 from 0 -> len(values).
        # Here is an example:
        # get_equals_row(1, [5, 6, 7, 8], 2) -> [0, 0, 0, 0, 5, 6, 7, 8, -5, -6, -7, -8, 0]
        values = get_values(derivative_terms, x)
        matrix.append(get_equals_row(x, values, number_of_spline_span))

        values = get_values(second_derivative_terms, x)
        matrix.append(get_equals_row(x, values, number_of_spline_span))

    # Finally let's do the 2 vectors we got supplied!
    values = get_values(derivative_terms, 0)
    matrix.append(get_full_row(0, values, number_of_spline_spans, start_velocity))

    values = get_values(derivative_terms, number_of_spline_spans)
    matrix.append(get_full_row(number_of_spline_spans, values, number_of_spline_spans, end_velocity))


def get_full_row(index, values, total_number_of_indexes, expected_value):
    """:returns: the full row of values (the rest of the values are 0 except from 0 -> len(values). Here is an example:
        get_full_row(1, [5, 6, 7, 8], 1) -> [0, 0, 0, 0, 5, 6, 7, 8]"""

    return_value = []

    for i in range(total_number_of_indexes):
        if i == index:
            return_value.extend(values)
            continue

        # Else add all the zeros
        for j in range(len(values)):
            return_value.append(0)

    return_value.append(expected_value)
    return return_value


def get_equals_row(start_index, values, total_number_of_indexes):
    """:returns: the full row of values so that values1 = values2 (the rest of the values are 0 except from 0 ->
        len(values). Here is an example:
        get_equals_row(1, [5, 6, 7, 8], [9, 10, 11, 12], 2) -> [0, 0, 0, 0, 5, 6, 7, 8, -9, -10, -11, -12, 0]"""

    return_value = []
    values2 = map(lambda x: -x, values2)  # Making values2 negative

    for i in range(total_number_of_indexes):
        if i == start_index:
            return_value.extend(values1)
            return_value.extend(values2)
            continue

        # Else add all the zeros
        for j in range(len(values)):
            return_value.append(0)

    return_value.append(expected_value)
    return return_value



def get_values(terms: list[Term], x_value):
    """Gets the equation values for the list of terms"""

    return_value = []

    for term in terms:
        value = term.coefficient * math.pow(x_value, term.degree)
        return_value.append(value)

    return return_value