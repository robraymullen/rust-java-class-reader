import java.io.Serializable;
import java.util.function.Function;

public class ClassFileCheck extends BaseCheckClass implements Serializable {
	
	public ClassFileCheck(String constructorMessage) {
		Function<String, String> msgWriter = (message) -> {
			return "MESSAGE: " + message;
		};
		System.out.println(msgWriter.apply(constructorMessage));
	}
	
	@Deprecated
	private static final long serialVersionUID = 1L;
	
	private void printNums(int x, int y) {
		int sum = x + y;
		System.out.println("Number 1: "+x+", Number 2:" + y + ", sum: "+sum);
	}
	
    @Override
	protected int add() {
		return 5 + 6;
	}
	
	

	public static void main(String[] args) {
		ClassFileCheck checker = new ClassFileCheck("Create the object");
		checker.add();
        checker.getBaseAdd(5, 4);
		checker.printNums(4,5);

	}

}
