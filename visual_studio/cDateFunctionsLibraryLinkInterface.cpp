#include "cDateFunctionsLibraryLink.h"
//#include "wstp.h"
#include "WolframLibrary.h"
#include <math.h>

DLLEXPORT mint WolframLibrary_getVersion() {
	return WolframLibraryVersion;
}

DLLEXPORT int WolframLibrary_initialize(WolframLibraryData libData) {
	return 0;
}

DLLEXPORT void WolframLibrary_uninitialize(WolframLibraryData libData) {
	return;
}


// WSTP version of date_list_link (return with second in float type) is almost 3 times slower than LibraryLink version with some interfacing on Mathematica side

//EXTERN_C DLLEXPORT int date_list_WS(WolframLibraryData libData, WSLINK link) {
//
//	double date;
//	int len;
//
//	if (!WSTestHead(link, "List", &len))
//		goto retPt;
//	if (len != 1)
//		goto retPt;
//
//	if (!WSGetReal(link, &date))
//		goto retPt;
//	if (!WSNewPacket(link))
//		goto retPt;
//
//	DateListResultWithoutSecond result = date_list_without_second(date);
//
//	/*if (!WSPutInteger(link, 34)) {
//		return LIBRARY_FUNCTION_ERROR;
//	}*/
//
//	WSPutFunction(link, "List", 6);
//	WSPutInteger(link, result.year);
//	WSPutInteger(link,result.month);
//	WSPutInteger(link, result.day);
//	WSPutInteger(link, result.hour);
//	WSPutInteger(link, result.minute);
//	WSPutReal(link,fmod(date,60.0));
//	WSPutFunction(link, "List", 0);
//	
//	return LIBRARY_NO_ERROR;
//retPt:
//	return LIBRARY_FUNCTION_ERROR;
//}


EXTERN_C DLLEXPORT int date_list_now_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {



	DateListResultWithoutSecond result = date_list_now();

	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;
}



EXTERN_C DLLEXPORT int second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {


	DateListResultWithoutSecond result = second_to_date_list(MArgument_getInteger(Args[0]));


	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;

}


EXTERN_C DLLEXPORT int unix_second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {


	DateListResultWithoutSecond result = unix_second_to_date_list(MArgument_getInteger(Args[0]));


	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;

}

EXTERN_C DLLEXPORT int julian_second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {


	DateListResultWithoutSecond result = julian_second_to_date_list(MArgument_getInteger(Args[0]));

	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;

}



EXTERN_C DLLEXPORT int date_list_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	mint* numbers;
	MTensor input = MArgument_getMTensor(Args[0]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	DateListResultWithoutSecond result = date_list_to_date_list((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]);

	mint dim[1];
	dim[0] = 5;

	MTensor output;
	libData->MTensor_new(MType_Integer, 1, dim, &output);

	mint counter = 1;
	libData->MTensor_setInteger(output, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(output, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(output, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(output, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(output, &counter, result.minute);

	MArgument_setMTensor(Res, output);

	return 0;
}

// get second argument to add to hour
EXTERN_C DLLEXPORT int date_list_to_date_list_extra_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {


	mint* numbers;

	numbers = libData->MTensor_getIntegerData(MArgument_getMTensor(Args[0]));

	DateListResultWithoutSecond result = date_list_to_date_list((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4] + MArgument_getInteger(Args[1]));

	mint dim[1];
	dim[0] = 5;

	MTensor output;
	libData->MTensor_new(MType_Integer, 1, dim, &output);

	mint counter = 1;
	libData->MTensor_setInteger(output, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(output, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(output, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(output, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(output, &counter, result.minute);

	MArgument_setMTensor(Res, output);

	return 0;
}



EXTERN_C DLLEXPORT int absolute_time_only_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	MArgument_setReal(Res, fmod(absolute_time_now(), 60.0));
	return 0;
}

EXTERN_C DLLEXPORT int absolute_time_now_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	MArgument_setReal(Res, absolute_time_now());
	return 0;
}

EXTERN_C DLLEXPORT int unix_time_now_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	MArgument_setReal(Res, unix_time_now());
	return 0;
}

EXTERN_C DLLEXPORT int julian_time_now_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	MArgument_setReal(Res, julian_time_now());
	return 0;
}

EXTERN_C DLLEXPORT int date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	// MTensor date;
	// date = MArgument_getMTensor(Args[0]);

	mint* numbers;

	MTensor input = MArgument_getMTensor(Args[0]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	MArgument_setInteger(Res, date_list_to_second((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
	return 0;
}

EXTERN_C DLLEXPORT int unix_date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	// MTensor date;
	// date = MArgument_getMTensor(Args[0]);

	mint* numbers;

	MTensor input = MArgument_getMTensor(Args[0]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	MArgument_setInteger(Res, unix_date_list_to_second((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
	return 0;
}

EXTERN_C DLLEXPORT int julian_date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	// MTensor date;
	// date = MArgument_getMTensor(Args[0]);

	mint* numbers;

	MTensor input = MArgument_getMTensor(Args[0]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	MArgument_setInteger(Res, julian_date_list_to_second((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
	return 0;
}


EXTERN_C DLLEXPORT int is_holiday_with_correction_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	MTensor input;
	mint* numbers;
	input = MArgument_getMTensor(Args[0]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
		break;
	}

	MArgument_setBoolean(Res, is_holiday_with_correction((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2]));
	return 0;
}



EXTERN_C DLLEXPORT int arithmetic_persian_second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	// mreal date = MArgument_getReal(Args[0]);

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);


	DateListResultWithoutSecond result = arithmetic_persian_second_to_date_list(cycle_id, MArgument_getInteger(Args[1]));



	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;

}


EXTERN_C DLLEXPORT int arithmetic_persian_date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	// MTensor date;
	// date = MArgument_getMTensor(Args[0]);

	mint* numbers;

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);


	MTensor input = MArgument_getMTensor(Args[1]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	MArgument_setInteger(Res, arithmetic_persian_date_list_to_second(cycle_id, (DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
	return 0;
}




//EXTERN_C DLLEXPORT int astronomical_persian_second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
//
//
//	DateListResultWithoutSecond result = astronomical_persian_second_to_date_list(MArgument_getInteger(Args[0]));
//
//	mint dim[1];
//	dim[0] = 5;
//
//	MTensor T0;
//	libData->MTensor_new(MType_Integer, 1, dim, &T0);
//
//	mint counter = 1;
//	libData->MTensor_setInteger(T0, &counter, result.year);
//	counter = 2;
//	libData->MTensor_setInteger(T0, &counter, result.month);
//	counter = 3;
//	libData->MTensor_setInteger(T0, &counter, result.day);
//	counter = 4;
//	libData->MTensor_setInteger(T0, &counter, result.hour);
//	counter = 5;
//	libData->MTensor_setInteger(T0, &counter, result.minute);
//
//	MArgument_setMTensor(Res, T0);
//
//	return 0;
//
//}


//EXTERN_C DLLEXPORT int astronomical_persian_date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
//	
//	mint* numbers;
//
//	MTensor input = MArgument_getMTensor(Args[0]);
//	numbers = libData->MTensor_getIntegerData(input);
//
//	switch (libData->MTensor_getFlattenedLength(input)) {
//	case 1:
//		numbers[1] = 1;
//	case 2:
//		numbers[2] = 1;
//	case 3:
//		numbers[3] = 0;
//	case 4:
//		numbers[4] = 0;
//		break;
//	}
//
//	MArgument_setInteger(Res, astronomical_persian_date_list_to_second((DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
//	return 0;
//}



EXTERN_C DLLEXPORT int islamic_second_to_date_list_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	// mreal date = MArgument_getReal(Args[0]);

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	DateListResultWithoutSecond result = islamic_second_to_date_list(cycle_id, MArgument_getInteger(Args[1]));


	mint dim[1];
	dim[0] = 5;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.year);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.month);
	counter = 3;
	libData->MTensor_setInteger(T0, &counter, result.day);
	counter = 4;
	libData->MTensor_setInteger(T0, &counter, result.hour);
	counter = 5;
	libData->MTensor_setInteger(T0, &counter, result.minute);

	MArgument_setMTensor(Res, T0);

	return 0;

}


EXTERN_C DLLEXPORT int islamic_date_list_to_second_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
	// MTensor date;
	// date = MArgument_getMTensor(Args[0]);

	mint* numbers;

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	MTensor input = MArgument_getMTensor(Args[1]);
	numbers = libData->MTensor_getIntegerData(input);

	switch (libData->MTensor_getFlattenedLength(input)) {
	case 1:
		numbers[1] = 1;
	case 2:
		numbers[2] = 1;
	case 3:
		numbers[3] = 0;
	case 4:
		numbers[4] = 0;
		break;
	}

	MArgument_setInteger(Res, islamic_date_list_to_second(cycle_id, (DateItemMaxType)numbers[0], (DateItemMaxType)numbers[1], (DateItemMaxType)numbers[2], (DateItemMaxType)numbers[3], (DateItemMaxType)numbers[4]));
	return 0;
}

EXTERN_C DLLEXPORT int gregorian_is_leap_year_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	MArgument_setBoolean(Res, gregorian_is_leap_year((YearType)MArgument_getInteger(Args[0])));
	return 0;
}

EXTERN_C DLLEXPORT int arithmetic_persian_is_leap_year_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	MArgument_setBoolean(Res, arithmetic_persian_is_leap_year((uint8_t)MArgument_getInteger(Args[0]), (YearType)MArgument_getInteger(Args[1])));
	return 0;
}

//EXTERN_C DLLEXPORT int astronomical_persian_is_leap_year_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
//
//	MArgument_setBoolean(Res, astronomical_persian_is_leap_year((YearType)MArgument_getInteger(Args[0])));
//	return 0;
//}

EXTERN_C DLLEXPORT int islamic_is_leap_year_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	MArgument_setBoolean(Res, islamic_is_leap_year((uint8_t)MArgument_getInteger(Args[0]), (YearType)MArgument_getInteger(Args[1])));
	return 0;
}

EXTERN_C DLLEXPORT int gregorian_month_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	MonthView result = gergorian_month_view(MArgument_getInteger(Args[0]));

	mint dim[1];
	dim[0] = 2;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.slot_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}

EXTERN_C DLLEXPORT int gregorian_year_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	YearView result = gergorian_year_view(MArgument_getInteger(Args[0]), MArgument_getInteger(Args[1]));

	mint dim[1];
	dim[0] = 13;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_start);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_2_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_3_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_4_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_5_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_6_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_7_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_8_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_9_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_10_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_11_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_12_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}


EXTERN_C DLLEXPORT int arithmetic_persian_month_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	MonthView result = arithmetic_persian_month_view(cycle_id, MArgument_getInteger(Args[1]));

	mint dim[1];
	dim[0] = 2;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.slot_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}

EXTERN_C DLLEXPORT int arithmetic_persian_year_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	YearView result = arithmetic_persian_year_view(cycle_id, MArgument_getInteger(Args[1]), MArgument_getInteger(Args[2]));

	mint dim[1];
	dim[0] = 13;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_start);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_2_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_3_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_4_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_5_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_6_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_7_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_8_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_9_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_10_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_11_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_12_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}


//EXTERN_C DLLEXPORT int astronomical_persian_month_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
//
//
//	MonthView result = astronomical_persian_month_view( MArgument_getInteger(Args[0]));
//
//	mint dim[1];
//	dim[0] = 2;
//
//	MTensor T0;
//	libData->MTensor_new(MType_Integer, 1, dim, &T0);
//
//	mint counter = 1;
//	libData->MTensor_setInteger(T0, &counter, result.offset);
//	counter = 2;
//	libData->MTensor_setInteger(T0, &counter, result.slot_length);
//
//	MArgument_setMTensor(Res, T0);
//
//	return 0;
//}
//
//EXTERN_C DLLEXPORT int astronomical_persian_year_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {
//
//	YearView result = astronomical_persian_year_view( MArgument_getInteger(Args[0]), MArgument_getInteger(Args[1]));
//
//	mint dim[1];
//	dim[0] = 13;
//
//	MTensor T0;
//	libData->MTensor_new(MType_Integer, 1, dim, &T0);
//
//	mint counter = 1;
//	libData->MTensor_setInteger(T0, &counter, result.offset);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_1_start);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_1_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_2_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_3_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_4_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_5_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_6_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_7_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_8_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_9_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_10_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_11_length);
//	counter++;
//	libData->MTensor_setInteger(T0, &counter, result.slot_12_length);
//
//	MArgument_setMTensor(Res, T0);
//
//	return 0;
//}



EXTERN_C DLLEXPORT int islamic_month_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	MonthView result = islamic_month_view(cycle_id, MArgument_getInteger(Args[1]));

	mint dim[1];
	dim[0] = 2;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter = 2;
	libData->MTensor_setInteger(T0, &counter, result.slot_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}

EXTERN_C DLLEXPORT int islamic_year_view_link(WolframLibraryData libData, mint Argc, MArgument* Args, MArgument Res) {

	uint8_t cycle_id;
	cycle_id = (uint8_t)MArgument_getInteger(Args[0]);

	YearView result = islamic_year_view(cycle_id, MArgument_getInteger(Args[1]), MArgument_getInteger(Args[2]));

	mint dim[1];
	dim[0] = 13;

	MTensor T0;
	libData->MTensor_new(MType_Integer, 1, dim, &T0);

	mint counter = 1;
	libData->MTensor_setInteger(T0, &counter, result.offset);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_start);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_1_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_2_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_3_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_4_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_5_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_6_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_7_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_8_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_9_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_10_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_11_length);
	counter++;
	libData->MTensor_setInteger(T0, &counter, result.slot_12_length);

	MArgument_setMTensor(Res, T0);

	return 0;
}
