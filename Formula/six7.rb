class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.57"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.57"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.57/six7-0.7.57-arm64-apple-darwin.tar.gz"
      sha256 "1d76750f640a9876dd9e5a8b254216c74afb726250c49c0eaf10984d72612b6a"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
